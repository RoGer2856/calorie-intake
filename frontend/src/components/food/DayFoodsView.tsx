import { ReactElement, useState } from "react";
import { IFoodResponse } from "../../messages/Food";
import FoodView from "./FoodView";
import { dayOfTheWeekToDayName, monthIndexToMonthName } from "../../utils/time";
import EditFoodForm, { IEditEvents } from "./EditFoodForm";
import { IUserInfoState } from "../../store/user-info";
import { useSelector } from "react-redux";
import { IUserInfo, Role } from "../../model/UserInfo";

export default function DayFoodsView(props: {
    month: number,
    dateOfMonth: number,
    dayOfTheWeek: number,
    foods: IFoodResponse[],
    onEditEvent: IEditEvents,
    userInfo: IUserInfo,
}): ReactElement {
    let [foodIdUnderEditing, setFoodIdUnderEditing] = useState<String | null>(null);

    const userInfo = useSelector((state: { userInfo: IUserInfoState }) => state.userInfo.userInfo);

    let calories = 0;
    for (const food of props.foods) {
        calories += food.calories;
    }

    const caloriesExceededMaximum = calories > props.userInfo.maxCaloriesPerDay;

    function editHandler(foodId: string) {
        setFoodIdUnderEditing(foodId);
    }

    const editEventHandler: IEditEvents = {
        onEdited: (id: string) => {
            setFoodIdUnderEditing(null);
            props.onEditEvent.onEdited(id);
        },
        onCancelled: (id: string) => {
            setFoodIdUnderEditing(null);
            props.onEditEvent.onCancelled(id);
        },
        onDeleted: (id: string) => {
            setFoodIdUnderEditing(null);
            props.onEditEvent.onDeleted(id);
        }
    };

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
                    if (foodIdUnderEditing === food.id) {
                        return (
                            <div key={food.id}>
                                <EditFoodForm
                                    food={food}
                                    onEditEvent={editEventHandler}
                                    userInfo={props.userInfo}
                                />
                            </div>
                        );
                    } else {
                        return (
                            <div className="card my-1" key={food.id}>
                                <ul className="list-group list-group-flush">
                                    <li className="list-group-item">
                                        <FoodView food={food} />
                                        {userInfo?.role === Role.Admin
                                            ?
                                            <button
                                                className="btn btn-primary"
                                                type="button"
                                                onClick={(e) => editHandler(food.id)}
                                            >
                                                Edit
                                            </button>
                                            :
                                            <></>
                                        }
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