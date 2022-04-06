import { ReactElement } from "react";
import { IMonthFoods, monthToSortedArray } from "../../model/Foods";
import { IUserInfo } from "../../model/UserInfo";
import { IEditEvents } from "./EditFoodForm";
import MonthFoodsView from "./MonthFoodsView";

export default function YearFoodsView(props: {
    year: number,
    foods: IMonthFoods[],
    onEditEvent: IEditEvents,
    userInfo: IUserInfo,
}): ReactElement {
    return (
        <>
            <div className="card m-2 p-2">
                <h1 className="card-header">Year: {props.year}</h1>
                {props.foods.map((month: IMonthFoods) => {
                    return (
                        <MonthFoodsView
                            userInfo={props.userInfo}
                            key={month.month}
                            month={month.month}
                            foods={monthToSortedArray(month)}
                            onEditEvent={props.onEditEvent}
                        />
                    );
                })}

            </div>
        </>
    );}