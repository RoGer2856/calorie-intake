import { ReactElement } from "react";
import { IAllFoods, allFoodsToSortedArray, IYearFoods, yearToSortedArray } from '../../model/Foods';
import { IEditEvents } from './EditFoodForm';
import YearFoodsView from './YearFoodsView';

export default function AllFoods(props: {
    maxCaloriesPerDay: number,
    allFoods: IAllFoods,
    onEditEvent: IEditEvents,
}): ReactElement {

    return (
        <>
            {allFoodsToSortedArray(props.allFoods).map((year: IYearFoods) => {
                return (
                    <YearFoodsView
                        key={year.year}
                        maxCaloriesPerDay={props.maxCaloriesPerDay}
                        year={year.year}
                        foods={yearToSortedArray(year)}
                        onEditEvent={props.onEditEvent}
                    />
                );
            })}
        </>
    );
}
