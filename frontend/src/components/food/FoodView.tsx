import { ReactElement } from "react";
import { IFoodResponse } from "../../messages/Food";

export default function FoodView(props: {
    food: IFoodResponse,
}): ReactElement {
    const dt = new Date(Date.parse(props.food.time));
    return (
        <>
            <p><b>{props.food.name}</b></p>
            <p>{props.food.calories} kcal</p>
            <p>{dt.toLocaleString()}</p>
        </>
    );
}
