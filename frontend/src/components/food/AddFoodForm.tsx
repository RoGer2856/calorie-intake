import { ReactElement } from "react";
import useApi from "../../hooks/use-api";
import useInput from "../../hooks/use-input";
import { IAddFoodResponse, IFoodRequest } from "../../messages/Food";
import { IUserInfo } from "../../model/UserInfo";
import { datetimeLocalInputToRfc3339 } from "../../utils/time";
import UseApiView from "../UseApiView";

export default function AddFoodForm(props: {
    onFoodAdded: (id: string) => void,
    userInfo: IUserInfo,
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

    async function submitHandler(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();

        let food: IFoodRequest = {
            name: nameInput.value,
            calories: parseInt(caloriesInput.value),
            time: datetimeLocalInputToRfc3339(timeInput.value),
        }

        let response = await api.addFood(props.userInfo.username, food);
        if (response !== null) {
            let data = response as IAddFoodResponse;
            props.onFoodAdded(data.id);

            nameInput.reset("");
            caloriesInput.reset("");
        }
    }

    return (
        <>
            <h1>Add food</h1>
            <form onSubmit={submitHandler}>
                <div>
                    <label
                        htmlFor="add-name"
                        className="form-label">What did you eat?</label>
                    <input
                        className="form-control"
                        type='text'
                        id='add-name'
                        value={nameInput.value}
                        onChange={nameInput.valueChangeHandler}
                        onBlur={nameInput.inputBlurHandler}
                        required
                    />
                </div>

                <div>
                    <label
                        htmlFor="add-calories"
                        className="form-label">How much calories did you eat?</label>
                    <input
                        className="form-control"
                        type='number'
                        id='add-calories'
                        value={caloriesInput.value}
                        onChange={caloriesInput.valueChangeHandler}
                        onBlur={caloriesInput.inputBlurHandler}
                        required
                    />
                </div>

                <div>
                    <label
                        htmlFor="add-time"
                        className="form-label">When did you eat it (UTC timezone)?</label>
                    <input
                        className="form-control"
                        type='datetime-local'
                        id='add-time'
                        value={timeInput.value}
                        onChange={timeInput.valueChangeHandler}
                        onBlur={timeInput.inputBlurHandler}
                        required
                    />
                </div>

                <button
                    className="btn btn-primary my-1"
                    type="submit"
                >
                    Add food
                </button>

                <UseApiView api={api}>
                    <></>
                </UseApiView>
            </form>
        </>
    );
}