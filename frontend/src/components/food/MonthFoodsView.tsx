import { ReactElement } from "react";
import { DayFoods } from "../../model/Foods";
import { monthIndexToMonthName } from "../../utils/time";
import DayFoodsView from "./DayFoodsView";
import { IEditEvents } from "./EditFoodForm";

export default function MonthFoodsView(props: {
    maxCaloriesPerDay: number,
    month: number,
    foods: DayFoods[],
    onEditEvent: IEditEvents,
}): ReactElement {
    return (
        <>
            <div className="card p-2 my-2">
                <h1 className="card-header">{monthIndexToMonthName(props.month)}</h1>
                <div className="d-flex flex-row flex-wrap p-1">
                    {props.foods.map((day: DayFoods) => {
                        return (
                            <DayFoodsView
                                key={day.dateOfMonth}
                                maxCaloriesPerDay={props.maxCaloriesPerDay}
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