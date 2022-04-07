import { ReactElement } from "react";
import { IFoodResponse } from "../../messages/Food";
import FoodView from "./FoodView";
import { dayOfTheWeekToDayName, monthIndexToMonthName } from "../../utils/time";
import { IEditEvents } from "./EditFoodForm";
import { IUserInfo } from "../../model/UserInfo";

export default function DayFoodsView(props: {
    month: number,
    dateOfMonth: number,
    dayOfTheWeek: number,
    foods: IFoodResponse[],
    onEditEvent: IEditEvents,
    userInfo: IUserInfo,
}): ReactElement {
    let calories = 0;
    for (const food of props.foods) {
        calories += food.calories;
    }

    const caloriesExceededMaximum = calories > props.userInfo.maxCaloriesPerDay;

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
                        Exceeded the daily consumption limit ({props.userInfo.maxCaloriesPerDay} kcal)
                    </div>
                    :
                    <></>}

                {props.foods.map((food: IFoodResponse) => {
                    return (
                        <div className="card my-1" key={food.id}>
                            <ul className="list-group list-group-flush">
                                <li className="list-group-item">
                                    <FoodView
                                        food={food}
                                        onEditEvent={props.onEditEvent}
                                        userInfo={props.userInfo}
                                    />
                                </li>
                            </ul>
                        </div>
                    );
                })}
            </div>
        </>
    );
}