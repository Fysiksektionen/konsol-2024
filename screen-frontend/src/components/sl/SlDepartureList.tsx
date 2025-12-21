import React from "react";

import SlData from "../../types/sl/SlData.ts";
import SlDepartureCard from "./SlDepartureCard.tsx";
import { SlTransportMode } from "../../types/sl/sl-types.ts";
import { SlDeparture } from "../../types/sl/SlDeparture.ts";

const TEKNISKA_HSK = 9204;

const SlDepartureList: React.FC<{ sl_data: SlData }> = ({ sl_data }) => {

  if (sl_data.departures.length === 0) { return <div className="sl-departure-list">Laddar tidtabell...</div>; }
  return <div className="sl-departure-list">
    <div className="sl-departure-list-metro">
      <h4>Tekniska Högskolan</h4>
      <SlDepartureCard departures={filter_departures(sl_data.departures, TEKNISKA_HSK, 1, SlTransportMode.Metro, 14)} />
      <SlDepartureCard departures={filter_departures(sl_data.departures, TEKNISKA_HSK, 2, SlTransportMode.Metro, 14)} />
      <h4>Roslagsbanan</h4>
      <SlDepartureCard departures={filter_departures(sl_data.departures, TEKNISKA_HSK, 2, SlTransportMode.Tram, 27)} />
      <SlDepartureCard departures={filter_departures(sl_data.departures, TEKNISKA_HSK, 2, SlTransportMode.Tram, 28)} />
      <SlDepartureCard departures={filter_departures(sl_data.departures, TEKNISKA_HSK, 2, SlTransportMode.Tram, 29)} />
    </div>
  </div>;
};

function filter_departures(departures: SlDeparture[], site_id: number, direction_code: number, transport_mode: SlTransportMode, line_id: number) {
  return departures.filter(dep => dep.site_id === site_id && dep.direction_code === direction_code && dep.transport_mode === transport_mode && dep.line_id == line_id);
}

export default SlDepartureList;
