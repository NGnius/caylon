import {
    gamepadDialogClasses,
    joinClassNames,
} from "decky-frontend-lib";

export const DISPLAY_KEY = "display";
export const VALUE_KEY = "value";

export const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);
