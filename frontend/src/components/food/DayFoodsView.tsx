import { ReactElement, useState } from "react";
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
    let [foodIdUnderEditing, setFoodIdUnderEditing] = useState<String | null>(null);

    let calories = 0;
    for (const food of props.foods) {
        calories += food.calories;
    }

    const caloriesExceededMaximum = calories > props.maxCaloriesPerDay;

    function saveHandler() {
        setFoodIdUnderEditing(null);
    }

    function editHandler(foodId: string) {
        setFoodIdUnderEditing(foodId);
    }

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
                    if (foodIdUnderEditing !== null) {
                        if (foodIdUnderEditing === food.id) {
                            return (
                                <div key={food.id}>
                                    <FoodView food={food} />
                                    <button onClick={saveHandler}>Save</button>
                                </div>
                            );
                        } else {
                            return (
                                <div key={food.id}>
                                    <FoodView food={food} />
                                </div>
                            );
                        }
                    } else {
                        return (
                            <div key={food.id}>
                                <FoodView food={food} />
                                <button onClick={(e) => editHandler(food.id)}>Edit</button>
                            </div>
                        );
                    }
                })}
            </div>
        </>
    );
}