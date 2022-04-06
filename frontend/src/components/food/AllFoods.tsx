import { ReactElement } from "react";
import { IAllFoods, allFoodsToSortedArray, IYearFoods, yearToSortedArray } from '../../model/Foods';
import { IUserInfo } from "../../model/UserInfo";
import { IEditEvents } from './EditFoodForm';
import YearFoodsView from './YearFoodsView';

export default function AllFoods(props: {
    allFoods: IAllFoods,
    onEditEvent: IEditEvents,
    userInfo: IUserInfo,
}): ReactElement {

    return (
        <>
            {allFoodsToSortedArray(props.allFoods).map((year: IYearFoods) => {
                return (
                    <YearFoodsView
                        key={year.year}
                        userInfo={props.userInfo}
                        year={year.year}
                        foods={yearToSortedArray(year)}
                        onEditEvent={props.onEditEvent}
                    />
                );
            })}
        </>
    );
}
