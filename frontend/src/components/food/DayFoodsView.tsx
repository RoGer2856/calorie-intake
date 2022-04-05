import { ReactElement, useState } from "react";
import { IFoodResponse } from "../../messages/Food";
import FoodView from "./FoodView";
import { dayOfTheWeekToDayName, monthIndexToMonthName } from "../../utils/time";
import EditFoodForm from "./EditFoodForm";

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

    function editHandler(foodId: string) {
        setFoodIdUnderEditing(foodId);
    }

    function foodEditedHandler(id: string) {
        setFoodIdUnderEditing(null);
    }

    function foodEditCancelledHandler(id: string) {
        setFoodIdUnderEditing(null);
    }

    function foodDeletedHandler(id: string) {
        setFoodIdUnderEditing(null);
    }

    return (
        <>
            <div className="card p-2 m-1">
                <div className="card-header">
                    <h1>{dayOfTheWeekToDayName(props.dayOfTheWeek)}</h1>
                    <p>{monthIndexToMonthName(props.month)} {props.dateOfMonth}</p>
                </div>
                {caloriesExceededMaximum
                    ?
                    <div className="alert alert-danger">
                        Exceeded the daily consumption limit ({props.maxCaloriesPerDay} kcal)
                    </div>
                    :
                    <></>}
                {props.foods.map((food: IFoodResponse) => {
                    if (foodIdUnderEditing === food.id) {
                        return (
                            <div key={food.id}>
                                <EditFoodForm
                                    food={food}
                                    onEdited={foodEditedHandler}
                                    onCancelled={foodEditCancelledHandler}
                                    onDeleted={foodDeletedHandler}
                                />
                            </div>
                        );
                    } else {
                        return (
                            <div className="card my-1" key={food.id}>
                                <ul className="list-group list-group-flush">
                                    <li className="list-group-item">
                                        <FoodView food={food} />
                                        <button
                                            className="btn btn-primary"
                                            type="button"
                                            onClick={(e) => editHandler(food.id)}
                                        >
                                            Edit
                                        </button>
                                    </li>
                                </ul>
                            </div>
                        );
                    }
                })}
            </div>
        </>
    );
}