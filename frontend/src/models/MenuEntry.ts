import { RouteObject } from "react-router-dom";

export default interface MenuEntry extends RouteObject {
    icon?: string;
    name?: string;
    hidden?: boolean;
}