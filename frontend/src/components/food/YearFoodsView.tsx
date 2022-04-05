import { ReactElement } from "react";
import { MonthFoods, monthToSortedArray } from "../../model/Foods";
import MonthFoodsView from "./MonthFoodsView";

export default function YearFoodsView(props: {
    maxCaloriesPerDay: number,
    year: number,
    foods: MonthFoods[],
}): ReactElement {
    return (
        <>
            <div className="card m-2 p-2">
                <h1 className="card-header">Year: {props.year}</h1>
                {props.foods.map((month: MonthFoods) => {
                    return (
                        <MonthFoodsView
                            maxCaloriesPerDay={props.maxCaloriesPerDay}
                            key={month.month}
                            month={month.month}
                            foods={monthToSortedArray(month)} />
                    );
                })}

            </div>
        </>
    );}