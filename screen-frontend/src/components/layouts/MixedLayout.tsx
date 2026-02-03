import SlData from "../../types/sl/SlData";
import { useState, useEffect } from "react";
import { SlideData } from "../../types/slides/SlideData";
import SlDepartureList from "../sl/SlDepartureList";
import Slideshow from "../slides/Slideshow";
import Calendar from "../Calendar";
import '../../styles/layouts/MixedLayout.css';
import fysikF from '../../assets/FrakturF2020.png';


function MixedLayout({ slides, sl_data }: { slides: SlideData[], sl_data: SlData }) {

  const [active, setActive] = useState("slide");

  useEffect(() => {
    const interval = setInterval(() => {
      setActive((prev) => (prev === "slide" ? "calendar" : "slide"));
    }, 15000);
    return () => clearInterval(interval); // cleanup on unmount
  }, []);

  return <>
    <div className="header">
      <img src={fysikF} alt="Fraktur F" className="fysikf" />
      <h1>KONSol</h1>
    </div>

    <div className='left'> {
      (active === "slide" && slides.length > 0) ? (
        <Slideshow slides={slides} />
      ) : (
        <div className="calendar-container">
          <Calendar
            calendarId="fysiksektionen.se_0187vbmdcivl8mtio142e23cas@group.calendar.google.com"
            apiKey={import.meta.env.VITE_GOOGLE_CALENDAR_API_KEY}
          />
        </div>
      )
    } </div>

    <div className="right">
      <p className="last-update">{`Senast uppdaterad: ${sl_data.last_update ? sl_data.last_update.toLocaleTimeString() : "Aldrig"}`}</p>
      <SlDepartureList sl_data={sl_data} />
    </div>
  </>;
}

export default MixedLayout;
