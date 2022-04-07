import { ReactElement, useEffect } from "react";
import useInput from "../../hooks/use-input";
import { IFoodResponse } from "../../messages/Food";

export default function FilterFoodsForm(props: {
	onFilter: (filterFunction: (food: IFoodResponse) => boolean) => void,
}): ReactElement {
	const fromTimeInput = useInput('', (value: String) => true);
	const toTimeInput = useInput('', (value: String) => true);

	useEffect(() => {
		if (fromTimeInput.value !== "" && toTimeInput.value !== "") {
			props.onFilter((food: IFoodResponse): boolean => {
				const fromDate = new Date(fromTimeInput.value);
				const toDate = new Date(toTimeInput.value);
				const foodDate = new Date(food.time);

				return foodDate >= fromDate && foodDate <= toDate;
			});
		} else if (fromTimeInput.value === "" && toTimeInput.value !== "") {
			props.onFilter((food: IFoodResponse): boolean => {
				const toDate = new Date(toTimeInput.value);
				const foodDate = new Date(food.time);

				return foodDate <= toDate;
			});
		} else if (fromTimeInput.value !== "" && toTimeInput.value === "") {
			props.onFilter((food: IFoodResponse): boolean => {
				const fromDate = new Date(fromTimeInput.value);
				const foodDate = new Date(food.time);

				return foodDate >= fromDate;
			});
		}
	}, [fromTimeInput.value, toTimeInput.value]);

	return (
		<>
			<h1>Filter foods</h1>
			<form>
				<div className="row p-1">
					<div className="col-2">
						<label
							htmlFor="filter-fromTime"
							className="form-label"
						>
							From time
						</label>
					</div>
					<div className="col-4">
						<input
							className="form-control"
							type='datetime-local'
							id='filter-fromTime'
							value={fromTimeInput.value}
							onChange={fromTimeInput.valueChangeHandler}
							onBlur={fromTimeInput.inputBlurHandler}
							required
						/>
					</div>
				</div>

				<div className="row p-1">
					<div className="col-2">
						<label
							htmlFor="filter-toTime"
							className="form-label"
						>
							To time
						</label>
					</div>
					<div className="col-4">
						<input
							className="form-control"
							type='datetime-local'
							id='filter-toTime'
							value={toTimeInput.value}
							onChange={toTimeInput.valueChangeHandler}
							onBlur={toTimeInput.inputBlurHandler}
							required
						/>
					</div>
				</div>
			</form>
		</>
	);
}