import { ReactElement } from "react";
import useApi from "../../hooks/use-api";
import useInput from "../../hooks/use-input";
import { IFoodResponse, IUpdateFoodRequest } from "../../messages/Food";
import { IUserInfo } from "../../model/UserInfo";
import { datetimeLocalInputToRfc3339, dateToDatetimeLocalInput } from "../../utils/time";
import UseApiView from "../UseApiView";

export interface IEditEvents {
    onEdited: (id: string) => void,
    onCancelled: (id: string) => void,
    onDeleted: (id: string) => void,
}

export default function EditFoodForm(props: {
    userInfo: IUserInfo,
    food: IFoodResponse,
    onEditEvent: IEditEvents,
}): ReactElement {
    const api = useApi();

    let nameInput = useInput(props.food.name, (name: string) => {
        return name.length !== 0;
    });

    let caloriesInput = useInput(props.food.calories.toString(), (calories: string) => {
        return true;
    });

    let timeInput = useInput(dateToDatetimeLocalInput(new Date(props.food.time)), (time: string) => {
        return true;
    });

    async function submitHandler(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();

        let food: IUpdateFoodRequest = {
            name: nameInput.value,
            calories: parseInt(caloriesInput.value),
            time: datetimeLocalInputToRfc3339(timeInput.value),
        }

        let response = await api.updateFood(props.userInfo.username, props.food.id, food);
        if (response !== null) {
            props.onEditEvent.onEdited(props.food.id);
        }
    }

    function cancelHandler() {
        props.onEditEvent.onCancelled(props.food.id);
    }

    async function deleteHandler() {
        let response = await api.deleteFood(props.userInfo.username, props.food.id);
        if (response !== null) {
            props.onEditEvent.onDeleted(props.food.id);
        }
    }

    return (
        <>
            <form onSubmit={submitHandler}>
                <div>
                    <label
                        htmlFor="edit-name"
                        className="form-label">What did you eat?</label>
                    <input
                        className="form-control"
                        type='text'
                        id='edit-name'
                        value={nameInput.value}
                        onChange={nameInput.valueChangeHandler}
                        onBlur={nameInput.inputBlurHandler}
                        required
                    />
                </div>

                <div>
                    <label
                        htmlFor="edit-calories"
                        className="form-label">How much calories did you eat?</label>
                    <input
                        className="form-control kcal"
                        type='number'
                        id='edit-calories'
                        value={caloriesInput.value}
                        onChange={caloriesInput.valueChangeHandler}
                        onBlur={caloriesInput.inputBlurHandler}
                        required
                    />
                </div>

                <div>
                    <label
                        htmlFor="edit-time"
                        className="form-label">When did you eat it (UTC timezone)?</label>
                    <input
                        className="form-control"
                        type='datetime-local'
                        id='edit-time'
                        value={timeInput.value}
                        onChange={timeInput.valueChangeHandler}
                        onBlur={timeInput.inputBlurHandler}
                        required
                    />
                </div>

                <button
                    className="btn btn-primary me-1 my-1"
                    type="button"
                    onClick={cancelHandler}
                >
                    Cancel
                </button>

                <button
                    className="btn btn-primary me-1 my-1"
                    type="submit"
                >
                    Save
                </button>

                <button
                    className="btn btn-primary me-1 my-1"
                    type="button"
                    onClick={deleteHandler}
                >
                    Delete
                </button>

                <UseApiView api={api}>
                    <></>
                </UseApiView>
            </form>
        </>
    );
}