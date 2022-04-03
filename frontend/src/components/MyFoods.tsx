import { useCallback, useEffect, useState } from 'react';
import { ReactElement } from "react";
import { ACCESS_TOKEN } from '../access_token';
import { IFoodResponse, IGetFoodListResponse } from '../messages/Food';
import { AllFoods, YearFoods } from '../model/GroupedFood';
import AddFoodForm from './AddFoodForm';
import FoodList from './FoodList';
import YearFoodsView from './YearFoodsView';

export default function (): ReactElement {
    const [foods, setFoods] = useState<IFoodResponse[]>([]);
    const [allFoods, setAllFoods] = useState<AllFoods>(new AllFoods());

    let fetchFoods = useCallback(async function () {
        let response: Response = await fetch(`/api/food?access_token=${ACCESS_TOKEN}`,
            {
                method: "GET"
            });

        if (response.ok) {
            let data = await response.json() as IGetFoodListResponse;
            data.foods.sort((a: IFoodResponse, b: IFoodResponse) => {
                const aDt = Date.parse(a.time.toString());
                const bDt = Date.parse(b.time.toString());
                return aDt < bDt ? 1 : -1;
            });
            setFoods(data.foods);

            let foods = new AllFoods();
            for (const food of data.foods) {
                foods.addFood(food);
            }
            setAllFoods(foods);
        } else {
        }
    }, []);

    useEffect(() => {
        fetchFoods();
    }, [fetchFoods]);

    async function foodAddedHandler(id: String) {
        fetchFoods();
        console.log(id);
    }

    return (
        <>
            <AddFoodForm onFoodAdded={foodAddedHandler} />

            {allFoods.toSortedArray().map((year: YearFoods) => {
                    return (
                        <YearFoodsView
                            key={year.year.toString()}
                            year={year.year}
                            foods={year.toSortedArray()}
                        />
                    );
                })}
        </>
    );
}
