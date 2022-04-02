import { ReactElement } from "react";
import { IFood } from "../messages/Food";

export default function FoodView(props: {
    food: IFood,
}): ReactElement {
    return (
        <>
            <hr />
            <p>{props.food.name}</p>
            <p>{props.food.calories}</p>
            <p>{props.food.time}</p>
        </>
    );
}
