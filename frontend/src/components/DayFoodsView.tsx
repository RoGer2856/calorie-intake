import { ReactElement } from "react";
import { IFoodResponse } from "../messages/Food";
import FoodView from "./FoodView";
import styles from "./DayFoodsView.module.css"

export default function DayFoodsView(props: {
    dateOfMonth: number,
    foods: IFoodResponse[],
}): ReactElement {
    return (
        <>
            <div className={styles.container.toString()}>
                <h1>Day: {props.dateOfMonth}</h1>
                {props.foods.map((food: IFoodResponse) => {
                    return (
                        <FoodView key={food.id} food={food} />
                    );
                })}

            </div>
        </>
    );
}