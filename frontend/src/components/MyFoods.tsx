import { useCallback, useEffect, useState } from 'react';
import { ReactElement } from "react";
import { ACCESS_TOKEN } from '../access_token';
import { IFoodResponse } from '../messages/Food';
import AddFoodForm from './AddFoodForm';
import FoodList from './FoodList';

export default function (): ReactElement {
    const [foods, setFoods] = useState([]);

    let fetchFoods = useCallback(async function () {
        let response: Response = await fetch(`/api/food?access_token=${ACCESS_TOKEN}`,
            {
                method: "GET"
            });

        if (response.ok) {
            let data = await response.json();
            data.foods.sort((a: IFoodResponse, b: IFoodResponse) => {
                const aDt = Date.parse(a.time.toString());
                const bDt = Date.parse(b.time.toString());
                return aDt < bDt ? 1 : -1;
            });
            setFoods(data.foods);
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
            <FoodList foods={foods} />
        </>
    );
}
