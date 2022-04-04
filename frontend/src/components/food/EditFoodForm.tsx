import { ReactElement, useEffect } from "react";
import useApi from "../../hooks/use-api";
import useInput from "../../hooks/use-input";
import { IFoodResponse, IUpdateFoodRequest } from "../../messages/Food";
import { datetimeLocalInputToRfc3339, dateToDatetimeLocalInput } from "../../utils/time";

export default function EditFoodForm(props: {
    food: IFoodResponse,
    onEdited: (id: string) => void,
    onCancelled: (id: string) => void,
    onDeleted: (id: string) => void,
}): ReactElement {
    const api = useApi();

    let nameInput = useInput('', (name: string) => {
        return name.length !== 0;
    });

    let caloriesInput = useInput('', (calories: string) => {
        return true;
    });

    let timeInput = useInput('', (time: string) => {
        return true;
    });

    useEffect(() => {
        nameInput.reset(props.food.name);
        caloriesInput.reset(props.food.calories.toString());
        timeInput.reset(dateToDatetimeLocalInput(new Date(props.food.time)));
    }, [props.food]);

    async function submitHandler(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();

        let food: IUpdateFoodRequest = {
            name: nameInput.value,
            calories: parseInt(caloriesInput.value),
            time: datetimeLocalInputToRfc3339(timeInput.value),
        }

        let response = await api.updateFood(props.food.id, food);
        if (response !== null) {
            props.onEdited(props.food.id);
        }
    }

    function cancelHandler() {
        props.onCancelled(props.food.id);
    }

    async function deleteHandler() {
        let response = await api.deleteFood(props.food.id);
        if (response !== null) {
            props.onDeleted(props.food.id);
        }
    }

    return (
        <>
            <form onSubmit={submitHandler}>
                <div>
                    <label
                        htmlFor="name"
                        className="form-label">What did you eat?</label>
                    <input
                        className="form-control"
                        type='text'
                        id='name'
                        value={nameInput.value}
                        onChange={nameInput.valueChangeHandler}
                        onBlur={nameInput.inputBlurHandler}
                        required
                    />
                </div>

                <div>
                    <label
                        htmlFor="calories"
                        className="form-label">How much calories did you eat?</label>
                    <input
                        className="form-control"
                        type='number'
                        id='calories'
                        value={caloriesInput.value}
                        onChange={caloriesInput.valueChangeHandler}
                        onBlur={caloriesInput.inputBlurHandler}
                        required
                    />
                    kcal
                </div>

                <div>
                    <label
                        htmlFor="time"
                        className="form-label">When did you eat it (UTC timezone)?</label>
                    <input
                        className="form-control"
                        type='datetime-local'
                        id='time'
                        value={timeInput.value}
                        onChange={timeInput.valueChangeHandler}
                        onBlur={timeInput.inputBlurHandler}
                        required
                    />
                </div>

                <button
                    className="btn btn-primary m-1"
                    type="button"
                    onClick={cancelHandler}
                >
                    Cancel
                </button>

                <button
                    className="btn btn-primary"
                    type="submit"
                >
                    Save
                </button>

                <button
                    className="btn btn-primary m-1"
                    type="button"
                    onClick={deleteHandler}
                >
                    Delete
                </button>

                {api.errorMessage === ""
                    ?
                    <></>
                    :
                    <p className="alert alert-danger">{api.errorMessage}</p>}
            </form>
        </>
    );
}