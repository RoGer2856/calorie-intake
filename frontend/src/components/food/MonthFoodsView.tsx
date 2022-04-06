import { ReactElement } from "react";
import { IDayFoods } from "../../model/Foods";
import { IUserInfo } from "../../model/UserInfo";
import { monthIndexToMonthName } from "../../utils/time";
import DayFoodsView from "./DayFoodsView";
import { IEditEvents } from "./EditFoodForm";

export default function MonthFoodsView(props: {
    month: number,
    foods: IDayFoods[],
    onEditEvent: IEditEvents,
    userInfo: IUserInfo,
}): ReactElement {
    return (
        <>
            <div className="card p-2 my-2">
                <h1 className="card-header">{monthIndexToMonthName(props.month)}</h1>
                <div className="d-flex flex-row flex-wrap p-1">
                    {props.foods.map((day: IDayFoods) => {
                        return (
                            <DayFoodsView
                                key={day.dateOfMonth}
                                userInfo={props.userInfo}
                                month={props.month}
                                dateOfMonth={day.dateOfMonth}
                                dayOfTheWeek={day.dayOfTheWeek}
                                foods={day.foods}
                                onEditEvent={props.onEditEvent}
                            />
                        );
                    })}

                </div>
            </div>
        </>
    );}