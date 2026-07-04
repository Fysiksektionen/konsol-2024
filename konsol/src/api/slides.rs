use crate::models::Slide;
use leptos::prelude::*;

#[server]
pub async fn get_slides() -> Result<Vec<Slide>, ServerFnError> {
    use crate::actions;
    use crate::db;

    let mut conn = db::get_conn()?;
    tokio::task::spawn_blocking(move || actions::get_all_slides(&mut conn))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(input = server_fn::codec::MultipartFormData)]
pub async fn upload_slide(data: server_fn::codec::MultipartData) -> Result<Slide, ServerFnError> {
    use crate::actions;
    use crate::db;
    use crate::fs_helpers;
    use chrono::{NaiveDate, NaiveTime};
    use uuid::Uuid;

    crate::session::require_auth().await?;

    let mut data = data
        .into_inner()
        .ok_or_else(|| ServerFnError::new("no multipart data"))?;

    let mut caption = None;
    let mut start = None;
    let mut end = None;
    let mut visible = None;
    let mut image_bytes: Option<Vec<u8>> = None;
    let mut image_ext: Option<String> = None;

    while let Ok(Some(mut field)) = data.next_field().await {
        let name = field.name().unwrap_or_default().to_string();

        if name == "imageFile" {
            image_ext = field.content_type().map(|m| m.subtype().to_string());
        }

        let mut bytes = Vec::new();
        while let Ok(Some(chunk)) = field.chunk().await {
            bytes.extend_from_slice(&chunk);
        }

        match name.as_str() {
            "imageFile" => image_bytes = Some(bytes),
            "caption" => caption = Some(String::from_utf8_lossy(&bytes).into_owned()),
            "start" => start = Some(String::from_utf8_lossy(&bytes).into_owned()),
            "end" => end = Some(String::from_utf8_lossy(&bytes).into_owned()),
            "visible" => visible = Some(String::from_utf8_lossy(&bytes).into_owned()),
            _ => {}
        }
    }

    let id = Uuid::new_v4();
    let caption = caption.ok_or_else(|| ServerFnError::new("missing caption"))?;
    let start = start.ok_or_else(|| ServerFnError::new("missing start date"))?;
    let end = end.ok_or_else(|| ServerFnError::new("missing end date"))?;
    // Matches the previous frontend: the "visible" checkbox is only present
    // in the form data when checked, so absence means false.
    let active = visible.as_deref() == Some("true");
    let filetype = image_ext.ok_or_else(|| ServerFnError::new("missing image file"))?;
    let image_bytes = image_bytes.ok_or_else(|| ServerFnError::new("missing image file"))?;

    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let slide = Slide {
        id: id.to_string(),
        caption,
        start_date: start
            .parse::<NaiveDate>()
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .and_time(midnight),
        end_date: end
            .parse::<NaiveDate>()
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .and_time(midnight),
        active,
        filetype,
    };

    let image_path = fs_helpers::save_image_bytes(&image_bytes, &id.to_string(), &slide.filetype).await?;

    let mut conn = db::get_conn()?;
    let slide_clone = slide.clone();
    let result = tokio::task::spawn_blocking(move || actions::insert_slide(&mut conn, slide_clone))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match result {
        Ok(added) => Ok(added),
        Err(e) => {
            let _ = fs_helpers::remove_file(image_path).await;
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

#[server]
pub async fn delete_slide(id: String) -> Result<(), ServerFnError> {
    use crate::actions;
    use crate::db;
    use crate::fs_helpers;
    use uuid::Uuid;

    crate::session::require_auth().await?;

    let uuid = Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
    let mut conn = db::get_conn()?;
    let slide = tokio::task::spawn_blocking(move || actions::pop_slide(&mut conn, &uuid))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    fs_helpers::remove_slide_image(&slide.id, &slide.filetype).await
}
