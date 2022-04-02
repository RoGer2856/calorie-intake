import { ReactElement } from "react";
import { IFood } from "../messages/Food";
import FoodView from "./FoodView";

export default function FoodList(props: {
    foods: IFood[],
}): ReactElement {
    return (
        <>
            <p>FoodList</p>

            {props.foods.map((food) =>
                <FoodView key={food.id as any} food={food} />
            )}
        </>
    )
}
