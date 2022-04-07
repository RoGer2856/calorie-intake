import { ReactElement } from "react";
import { IGetFoodReportResponse } from "../../messages/Food";

export default function FoodReport(props: {
	foodReport: IGetFoodReportResponse,
}): ReactElement {
	return (
		<>
			<div className="card">
				<div className="card-header">
					Report
				</div>
				<ul className="list-group list-group-flush">
					<li className="list-group-item">Average calories consumed last week: {props.foodReport.average_calories_consumed_last_week}</li>
					<li className="list-group-item">Food entries added last week: {props.foodReport.food_entries_added_last_week}</li>
					<li className="list-group-item">Food entries added the week before last week: {props.foodReport.food_entries_added_week_before_last_week}</li>
				</ul>
			</div>
		</>
	);
}