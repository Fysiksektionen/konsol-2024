import React from "react";

import "../../styles/sl/SlDepartureCard.css";

import SlLineBadge from "./SlLineBadge.tsx";
import { SlDeparture } from "../../types/sl/SlDeparture.ts";

// Only show `DEPARTURE_COUNT` number of departures that are more than `TIME_MARGIN_MS` ms in the future
const TIME_MARGIN_MS = 6 * 60 * 1000;
const DEPARTURE_COUNT = 4;

const SlDepartureCard: React.FC<{ departures: SlDeparture[] }> = ({ departures }) => {
  if (departures.length === 0) { return <div className="sl-departure-card">---</div>; }
  // Filter according to `TIME_MARGIN_MS` and limit to `DEPARTURE_COUNT`
  departures = departures.filter(dep => dep.expected_time.getTime() >= new Date().getTime() + TIME_MARGIN_MS).slice(0, DEPARTURE_COUNT);
  const firstDeparture = departures[0];
  return (
    <div className="sl-departure-card">

      <div className="sl-departure-card-top">

        <SlLineBadge mode={firstDeparture.transport_mode} line_group={firstDeparture.line_group} line_designation={firstDeparture.line_designation} />
        <div className="sl-destination">{firstDeparture.destination}</div>
      </div>
      <div className="sl-departure-card-bottom">
        <div className="sl-next-departure">{firstDeparture.display_time}</div>
        <div className="sl-future-departures">{departures.slice(1).map(dep => dep.display_time).join(", ")}</div>
      </div>
    </div>
  )
};

export default SlDepartureCard;