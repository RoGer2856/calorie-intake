import { ReactElement } from "react";
import { IFoodResponse } from "../messages/Food";

export default function FoodView(props: {
    food: IFoodResponse,
}): ReactElement {
    const dt = new Date(Date.parse(props.food.time.toString()));
    return (
        <>
            <hr />
            <p>{props.food.name}</p>
            <p>{props.food.calories}</p>
            <p>{dt.toLocaleString()}</p>
        </>
    );
}
