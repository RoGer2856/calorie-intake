import { ReactElement } from "react";
import { DayFoods } from "../../model/Foods";
import { monthIndexToMonthName } from "../../utils/time";
import DayFoodsView from "./DayFoodsView";
import styles from "./MonthFoodsView.module.css"

export default function MonthFoodsView(props: {
    maxCaloriesPerDay: number,
    month: number,
    foods: DayFoods[],
}): ReactElement {
    return (
        <>
            <div className={styles.frame.toString()}>
                <h1>{monthIndexToMonthName(props.month)}</h1>
                <div className={styles.container.toString()}>
                    {props.foods.map((day: DayFoods) => {
                        return (
                            <DayFoodsView
                                key={day.dateOfMonth}
                                maxCaloriesPerDay={props.maxCaloriesPerDay}
                                month={props.month}
                                dateOfMonth={day.dateOfMonth}
                                dayOfTheWeek={day.dayOfTheWeek}
                                foods={day.foods} />
                        );
                    })}

                </div>
            </div>
        </>
    );}