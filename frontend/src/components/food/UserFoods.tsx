import { useCallback, useEffect, useState } from 'react';
import { ReactElement } from "react";
import useApi from '../../hooks/use-api';
import { IFoodResponse } from '../../messages/Food';
import { addFoodToAll, createAllFoods } from '../../model/Foods';
import { IUserInfo } from '../../model/UserInfo';
import UseApiView from '../UseApiView';
import AddFoodForm from './AddFoodForm';
import AllFoods from './AllFoods';
import { IEditEvents } from './EditFoodForm';
import FilterFoodsForm from './FilterFoodsForm';

interface IFilterWrapper {
    filterFunc: (food: IFoodResponse) => boolean,
}

export default function UserFoods(props: {
    userInfo: IUserInfo,
}): ReactElement {
    const api = useApi();

    const [allFoods, setAllFoods] = useState<IFoodResponse[]>([]);
    const [filteredFoods, setFilteredFoods] = useState(createAllFoods(null));
    const [showAddFood, setShowAddFood] = useState(false);
    const [filterFunc, setFilterFunc] = useState<IFilterWrapper>({
        filterFunc: (food: IFoodResponse): boolean => {
            return true;
        }
    });

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
            setAllFoods(response.foods);
        }
    }, [props.userInfo]);

    useEffect(() => {
        fetchFoods();
    }, [fetchFoods, props.userInfo]);

    useEffect(() => {
        let filteredFoodsFoods = createAllFoods(null);
        allFoods.filter(filterFunc.filterFunc).forEach((food: IFoodResponse) => {
            addFoodToAll(filteredFoodsFoods, food);
        })
        setFilteredFoods(filteredFoodsFoods);
    }, [allFoods, filterFunc]);

    async function foodAddedHandler(id: string) {
        let response = await api.getFoodList(props.userInfo.username);
        if (response !== null) {
            setAllFoods(response.foods);
        }
    }

    function showAddFoodHandler() {
        setShowAddFood(true);
    }

    function hideAddFoodHandler() {
        setShowAddFood(false);
    }

    function filterHandler(filterFunction: (food: IFoodResponse) => boolean) {
        setFilterFunc({
            filterFunc: filterFunction,
        });
    }

    return (
        <>
            <UseApiView api={api} >
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

                <div className="card p-2 m-2">
                    <FilterFoodsForm onFilter={filterHandler} />
                </div>

                <AllFoods
                    userInfo={props.userInfo}
                    allFoods={filteredFoods}
                    onEditEvent={editEventHandler}
                />
            </UseApiView>
        </>
    );
}
