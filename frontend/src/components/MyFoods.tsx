import { useCallback, useEffect, useState } from 'react';
import { ReactElement } from "react";
import { ACCESS_TOKEN } from '../access_token';
import useApi from '../hooks/use-api';
import { IFoodResponse, IGetFoodListResponse } from '../messages/Food';
import { AllFoods, YearFoods } from '../model/GroupedFood';
import AddFoodForm from './AddFoodForm';
import YearFoodsView from './YearFoodsView';

export default function (): ReactElement {
    const api = useApi();

    const [allFoods, setAllFoods] = useState<AllFoods>(new AllFoods());

    let fetchFoods = useCallback(async function () {
        let response = await api.getFoodList();
        if (response !== null) {
            let data = response as IGetFoodListResponse;
            let foods = new AllFoods();
            for (const food of data.foods) {
                foods.addFood(food);
            }
            setAllFoods(foods);
        }
    }, []);

    useEffect(() => {
        fetchFoods();
    }, [fetchFoods]);

    async function foodAddedHandler(id: string) {
        fetchFoods();
        console.log(id);
    }

    return (
        <>
            <AddFoodForm onFoodAdded={foodAddedHandler} />

            {allFoods.toSortedArray().map((year: YearFoods) => {
                    return (
                        <YearFoodsView
                            key={year.year}
                            year={year.year}
                            foods={year.toSortedArray()}
                        />
                    );
                })}
        </>
    );
}