import { ReactElement } from "react";
import { DayFoods } from "../model/GroupedFood";
import DayFoodsView from "./DayFoodsView";
import styles from "./MonthFoodsView.module.css"

export default function MonthFoodsView(props: {
    month: number,
    foods: DayFoods[],
}): ReactElement {
    return (
        <>
            <div className={styles.frame.toString()}>
                <h1>Month: {props.month}</h1>
                <div className={styles.container.toString()}>
                    {props.foods.map((day: DayFoods) => {
                        return (
                            <DayFoodsView key={day.dateOfMonth} dateOfMonth={day.dateOfMonth} foods={day.foods} />
                        );
                    })}

                </div>
            </div>
        </>
    );}