import { ReactElement } from "react";
import { ACCESS_TOKEN } from "../access_token";
import useInput from "../hooks/use-input";
import { IFoodRequest } from "../messages/Food";
import { datetimeLocalInputToRfc3339 } from "../utils/time";

export default function AddFoodForm(props: {
    onFoodAdded: (id: String) => void,
}): ReactElement {
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

        let response: Response = await fetch(`/api/food?access_token=${ACCESS_TOKEN}`,
            {
                method: "POST",
                body: JSON.stringify(food),
            });

        if (response.ok) {
            nameInput.reset("");
            caloriesInput.reset("");

            const data = await response.json();
            props.onFoodAdded(data.id);
        } else {
        }
    }

    return (
        <>
            <h1>Add food</h1>
            <form onSubmit={submitHandler}>
                <div>
                    <label htmlFor="name">What did you eat?</label>
                    <input
                        type='text'
                        id='name'
                        value={nameInput.value}
                        onChange={nameInput.valueChangeHandler}
                        onBlur={nameInput.inputBlurHandler}
                        required
                    />
                </div>

                <div>
                    <label>How much calories did you eat?</label>
                    <input
                        type='number'
                        id='calories'
                        value={caloriesInput.value}
                        onChange={caloriesInput.valueChangeHandler}
                        onBlur={caloriesInput.inputBlurHandler}
                        required
                    />
                </div>

                <div>
                    <label>When did you eat it?</label>
                    <input
                        type='datetime-local'
                        id='time'
                        value={timeInput.value}
                        onChange={timeInput.valueChangeHandler}
                        onBlur={timeInput.inputBlurHandler}
                        required
                    />
                </div>

                <button type="submit">Add food</button>
            </form>
        </>
    );
}