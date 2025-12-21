import React from "react";

import "../../styles/sl/SlLineBadge.css";
import "../../styles/sl/SlDepartureCard.css";

import {SlLineGroup, SlTransportMode} from "../../types/sl/SlDeparture.ts";

// SVG:s shamelessly taken from sl.se Sök avgångar
const icon_size = "2vw";

const train_icon = <svg
    xmlns="http://www.w3.org/2000/svg"
    width={icon_size}
    height={icon_size}
    fill="none"
    viewBox="0 0 24 24">
    <rect
        width="24"
        height="24"
        fill="var(--color-dark-gray)"
        rx="4"></rect>
    <path
        fill="var(--color-white)"
        fill-rule="evenodd"
        d="M12 19a7 7 0 100-14 7 7 0 000 14zm8-7a8 8 0 11-16 0 8 8 0 0116 0zm-6.75-1.25h3.25v-2.5h-9v2.5h3.25v6.75h2.5v-6.75z"
        clip-rule="evenodd"></path>
</svg>;

const metro_icon = <svg
    xmlns="http://www.w3.org/2000/svg"
    width={icon_size}
    height={icon_size}
    fill="none"
    viewBox="0 0 24 24">
    <rect
        width="24"
        height="24"
        fill="var(--color-dark-gray)"
        rx="4"></rect>
    <path
        fill="var(--color-white)"
        fill-rule="evenodd"
        d="M12 19a7 7 0 100-14 7 7 0 000 14zm8-7a8 8 0 11-16 0 8 8 0 0116 0zm-6.75-1.25h3.25v-2.5h-9v2.5h3.25v6.75h2.5v-6.75z"
        clip-rule="evenodd"></path>
</svg>;

const bus_icon = <svg
    xmlns="http://www.w3.org/2000/svg"
    width={icon_size}
    height={icon_size}
    fill="none"
    viewBox="0 0 24 24">
    <rect
        width="24"
        height="24"
        fill="var(--color-dark-gray)"
        rx="4"></rect>
    <path
        fill="var(--color-white)"
        fill-rule="evenodd"
        d="M6 6a2 2 0 012-2h8a2 2 0 012 2v11a2 2 0 01-1 1.732v.018a1.25 1.25 0 01-2.475.25h-5.05A1.25 1.25 0 017 18.75v-.018A2 2 0 016 17V6zm1 2a1 1 0 011-1h8a1 1 0 011 1v5.5a2 2 0 01-2 2H9a2 2 0 01-2-2V8zm2.5-3a.5.5 0 000 1h5a.5.5 0 000-1h-5zM15 17a.5.5 0 01.5-.5h1a.5.5 0 01.5.5v.5a.5.5 0 01-.5.5h-1a.5.5 0 01-.5-.5V17zm-7.5-.5a.5.5 0 00-.5.5v.5a.5.5 0 00.5.5h1a.5.5 0 00.5-.5V17a.5.5 0 00-.5-.5h-1z"
        clip-rule="evenodd"></path>
</svg>;

const tram_icon = train_icon; // identical

const ferry_icon = <svg
    xmlns="http://www.w3.org/2000/svg"
    width={icon_size}
    height={icon_size}
    fill="none"
    viewBox="0 0 24 24">
    <rect
        width="24"
        height="24"
        fill="var(--color-dark-gray)"
        rx="4"></rect>
    <path
        fill="var(--color-white)"
        fill-rule="evenodd"
        d="M11 5v-.777c0-.075.078-.14.19-.158a5.083 5.083 0 011.62 0c.112.018.19.083.19.158V5h-2zM6.504 15L6.5 16c.45 0 .697-.101.97-.212.334-.136.706-.288 1.531-.288.826 0 1.348.303 1.818.576.383.222.732.424 1.181.424.45 0 .798-.202 1.183-.424.47-.273.994-.576 1.819-.576s1.196.152 1.53.288c.272.111.519.212.968.212v-1c.002 0 .882-1.993 1.295-2.927a.499.499 0 00-.273-.665L17.5 11V7.5a2 2 0 00-2-2H8.502a2 2 0 00-2 1.999L6.5 11l-1.019.408a.5.5 0 00-.271.666L6.504 15zm1.43 4.368c-.539.308-1.21.632-2.434.632H4v-2H5.5c.776 0 1.105-.176 1.441-.368l.095-.055C7.459 17.332 8.032 17 9 17c1.05 0 1.63.385 2.055.668l.004.003c.323.215.494.329.941.329.456 0 .647-.12.98-.338l.02-.013c.414-.27.994-.649 2-.649.928 0 1.503.329 1.916.565l.11.063c.346.194.694.372 1.474.372H20v2H18.5c-1.22 0-1.906-.322-2.452-.628l-.1-.056c-.396-.223-.56-.316-.948-.316-.41 0-.577.109-.928.338-.435.283-1.028.662-2.072.662-1.05 0-1.63-.385-2.055-.668l-.004-.003C9.618 19.114 9.447 19 9 19c-.425 0-.598.1-.98.319l-.086.05zM16 7H8v3.5L12 9l4 1.5V7z"
        clip-rule="evenodd"></path>
</svg>;

const ship_icon = ferry_icon; // identical
const taxi_icon = bus_icon; // placeholder

const get_svg = (mode: SlTransportMode) => {
    switch (mode) {
        case SlTransportMode.Train:
            return train_icon;
        case SlTransportMode.Metro:
            return metro_icon;
        case SlTransportMode.Bus:
            return bus_icon;
        case SlTransportMode.Tram:
            return tram_icon;
        case SlTransportMode.Ferry:
            return ferry_icon;
        case SlTransportMode.Ship:
            return ship_icon;
        case SlTransportMode.Taxi:
            return taxi_icon;
        default:
            throw new Error(`Unexpected transport mode ${mode}`);
    }
};

const get_badge_class = (line_group: SlLineGroup) => {
    switch (line_group) {
        case SlLineGroup.Train:
            return "sl-line-badge-train";
        case SlLineGroup.RedMetro:
            return "sl-line-badge-red-metro";
        case SlLineGroup.GreenMetro:
            return "sl-line-badge-green-metro";
        case SlLineGroup.BlueMetro:
            return "sl-line-badge-blue-metro";
        case SlLineGroup.Bus:
            return "sl-line-badge-bus";
        case SlLineGroup.BlueBus:
            return "sl-line-badge-blue-bus";
        case SlLineGroup.CityLine:
            return "sl-line-badge-city-line";
        case SlLineGroup.RoslagenLine:
            return "sl-line-badge-roslagen-line";
        default:
            throw new Error(`Unexpected line group ${line_group}`);
    }
};

const SlLineBadge: React.FC<{ mode: SlTransportMode, line_group: SlLineGroup, line_designation: string }> = ({mode, line_group, line_designation}) => {
    return <div className={`sl-line-badge ${get_badge_class(line_group)}`}>
        <div className="sl-line-number">
            <p style={{fontSize:"1vw", paddingRight:"0.5vw"}}>{line_designation}</p>
        </div>
        {get_svg(mode)}
    </div>;
};

export default SlLineBadge;
