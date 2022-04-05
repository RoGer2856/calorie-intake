import { useCallback, useEffect, useState } from 'react';
import { ReactElement } from "react";
import useApi from '../../hooks/use-api';
import { createAllFoods } from '../../model/Foods';
import AddFoodForm from './AddFoodForm';
import AllFoods from './AllFoods';
import { IEditEvents } from './EditFoodForm';

export default function MyFoods(props: {
    maxCaloriesPerDay: number,
}): ReactElement {
    const api = useApi();

    const [foods, setFoods] = useState(createAllFoods(null));
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
            setFoods(createAllFoods(response!.foods));
        }
    }, []);

    useEffect(() => {
        fetchFoods();
    }, [fetchFoods]);

    async function foodAddedHandler(id: string) {
        let response = await api.getFoodList();
        if (response !== null) {
            setFoods(createAllFoods(response!.foods));
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

            <AllFoods
                maxCaloriesPerDay={props.maxCaloriesPerDay}
                allFoods={foods}
                onEditEvent={editEventHandler}
            />
        </>
    );
}
