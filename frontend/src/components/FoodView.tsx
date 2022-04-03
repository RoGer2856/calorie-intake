import { ReactElement } from "react";
import { IFoodResponse } from "../messages/Food";
import styles from "./FoodView.module.css"

export default function FoodView(props: {
    food: IFoodResponse,
}): ReactElement {
    const dt = new Date(Date.parse(props.food.time.toString()));
    return (
        <>
            <div className={styles.container.toString()}>
                <p>{props.food.name}</p>
                <p>{props.food.calories}</p>
                <p>{dt.toLocaleString()}</p>
            </div>
        </>
    );
}
