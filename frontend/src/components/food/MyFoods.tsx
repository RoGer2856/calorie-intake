import { useCallback, useEffect, useState } from 'react';
import { ReactElement } from "react";
import useApi from '../../hooks/use-api';
import { addFoodToAll, AllFoods, allFoodsToSortedArray, createAllFoods, YearFoods, yearToSortedArray } from '../../model/Foods';
import AddFoodForm from './AddFoodForm';
import { IEditEvents } from './EditFoodForm';
import YearFoodsView from './YearFoodsView';

export default function MyFoods(props: {
    maxCaloriesPerDay: number,
}): ReactElement {
    const api = useApi();

    const [foods, setFoods] = useState(createAllFoods());
    const [showAddFood, setShowAddFood] = useState(false);

    const editEventHandler: IEditEvents = {
        onEdited: async (id: String) => {
            await fetchFoods();
        },
        onCancelled: (id: String) => {
        },
        onDeleted: async (id: String) => {
            await fetchFoods();
        }
    };

    let fetchFoods = useCallback(async function () {
        let response = await api.getFoodList();
        if (response !== null) {
            setFoods((state: AllFoods) => {
                let foods = createAllFoods();
                for (const food of response!.foods) {
                    addFoodToAll(foods, food);
                }
                return foods;
            })
        }
    }, []);

    useEffect(() => {
        fetchFoods();
    }, [fetchFoods]);

    async function foodAddedHandler(id: string) {
        let response = await api.getFoodList();
        if (response !== null) {
            setFoods((state: AllFoods) => {
                let foods = createAllFoods();
                for (const food of response!.foods) {
                    addFoodToAll(foods, food);
                }
                return foods;
            })
        }
    }

    function showAddFoodHandler() {
        setShowAddFood(true);
    }

    function hideAddFoodHandler() {
        setShowAddFood(false);
    }

    return (
        <>
            {showAddFood
                ?
                <>
                    <button
                        className="btn btn-primary"
                        onClick={hideAddFoodHandler}
                    >
                        Close
                    </button>
                    <div className="card p-2 m-2">
                        <AddFoodForm onFoodAdded={foodAddedHandler} />
                    </div>
                </>
                :
                <button
                    className="btn btn-primary"
                    onClick={showAddFoodHandler}
                >
                    Add food
                </button>}

            {allFoodsToSortedArray(foods).map((year: YearFoods) => {
                return (
                    <YearFoodsView
                        key={year.year}
                        maxCaloriesPerDay={props.maxCaloriesPerDay}
                        year={year.year}
                        foods={yearToSortedArray(year)}
                        onEditEvent={editEventHandler}
                    />
                );
            })}
        </>
    );
}
