import { useCallback, useEffect, useState } from 'react';
import { ReactElement } from "react";
import useApi from '../../hooks/use-api';
import { createAllFoods } from '../../model/Foods';
import { IUserInfo } from '../../model/UserInfo';
import UseApiView from '../UseApiView';
import AddFoodForm from './AddFoodForm';
import AllFoods from './AllFoods';
import { IEditEvents } from './EditFoodForm';

export default function MyFoods(props: {
    userInfo: IUserInfo,
}): ReactElement {
    const [apiFeedback, api] = useApi();

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
        let response = await api.getFoodList(props.userInfo.username);
        if (response !== null) {
            setFoods(createAllFoods(response!.foods));
        }
    }, [props.userInfo, api]);

    useEffect(() => {
        fetchFoods();
    }, [fetchFoods, props.userInfo]);

    async function foodAddedHandler(id: string) {
        let response = await api.getFoodList(props.userInfo.username);
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
            <UseApiView apiFeedback={apiFeedback} >
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
                            <AddFoodForm
                                onFoodAdded={foodAddedHandler}
                                userInfo={props.userInfo}
                            />
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
                    userInfo={props.userInfo}
                    allFoods={foods}
                    onEditEvent={editEventHandler}
                />
            </UseApiView>
        </>
    );
}
