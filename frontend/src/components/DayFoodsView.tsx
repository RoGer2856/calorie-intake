import { ReactElement } from "react";
import { IFoodResponse } from "../messages/Food";
import FoodView from "./FoodView";
import styles from "./DayFoodsView.module.css"
import { dayOfTheWeekToDayName, monthIndexToMonthName } from "../utils/time";

export default function DayFoodsView(props: {
    month: number,
    dateOfMonth: number,
    dayOfTheWeek: number,
    foods: IFoodResponse[],
}): ReactElement {
    return (
        <>
            <div className={styles.container.toString()}>
                <h1>{dayOfTheWeekToDayName(props.dayOfTheWeek)}</h1>
                <p>{monthIndexToMonthName(props.month)} {props.dateOfMonth}</p>
                {props.foods.map((food: IFoodResponse) => {
                    return (
                        <FoodView key={food.id} food={food} />
                    );
                })}
            </div>
        </>
    );
}