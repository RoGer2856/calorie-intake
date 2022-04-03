import { ReactElement } from "react";
import { MonthFoods } from "../model/Foods";
import styles from "./YearFoodsView.module.css"
import MonthFoodsView from "./MonthFoodsView";

export default function YearFoodsView(props: {
    maxCaloriesPerDay: number,
    year: number,
    foods: MonthFoods[],
}): ReactElement {
    return (
        <>
            <div className={styles.container.toString()}>
                <h1>Year: {props.year}</h1>
                {props.foods.map((month: MonthFoods) => {
                    return (
                        <MonthFoodsView
                            maxCaloriesPerDay={props.maxCaloriesPerDay}
                            key={month.month}
                            month={month.month}
                            foods={month.toSortedArray()} />
                    );
                })}

            </div>
        </>
    );}