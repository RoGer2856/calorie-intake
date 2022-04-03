import { ReactElement } from "react";
import { IFoodResponse } from "../../messages/Food";
import FoodView from "./FoodView";
import styles from "./DayFoodsView.module.css"
import { dayOfTheWeekToDayName, monthIndexToMonthName } from "../../utils/time";

export default function DayFoodsView(props: {
    maxCaloriesPerDay: number,
    month: number,
    dateOfMonth: number,
    dayOfTheWeek: number,
    foods: IFoodResponse[],
}): ReactElement {
    let calories = 0;
    for (const food of props.foods) {
        calories += food.calories;
    }

    const caloriesExceededMaximum = calories > props.maxCaloriesPerDay;
console.log(props.maxCaloriesPerDay);
    return (
        <>
            <div className={styles.container.toString()}>
                {caloriesExceededMaximum
                    ?
                    <p>You consumed more than {props.maxCaloriesPerDay} kcal</p>
                    :
                    <></>}
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