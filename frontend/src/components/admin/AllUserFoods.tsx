import React, { ReactElement, useCallback, useEffect, useState } from "react";
import { useSelector } from "react-redux";
import useApi from "../../hooks/use-api";
import { createAllFoods } from "../../model/Foods";
import { IUserInfo } from "../../model/UserInfo";
import { IUserInfoState } from "../../store/user-info";
import AllFoods from "../food/AllFoods";
import { IEditEvents } from "../food/EditFoodForm";

export default function AllUserFoods(): ReactElement {
	const api = useApi();

	const myUserInfo = useSelector((state: { userInfo: IUserInfoState }) => state.userInfo.userInfo);

	const [userInfos, setUserInfos] = useState<IUserInfo[]>([]);
	const [foods, setFoods] = useState(createAllFoods(null));
	const [selectedUserInfo, setSelectedUserInfo] = useState(myUserInfo);

	let fetchUserInfos = useCallback(async () => {
		const response = await api.getUserList();
		if (response !== null) {
			setUserInfos(response);
		}
	}, []);

	let fetchDataForUser = useCallback(async (userInfo: IUserInfo) => {
		setSelectedUserInfo(userInfo);

		let response = await api.getFoodListOf(userInfo.username);
		if (response !== null) {
			setFoods(createAllFoods(response!.foods));
		}
	}, []);

	useEffect(() => {
		fetchDataForUser(myUserInfo!);
		fetchUserInfos();
	}, [fetchDataForUser, fetchUserInfos])

	async function selectHandler(event: React.FormEvent<HTMLSelectElement>) {
		const username = event.currentTarget.value;
		const userInfo = userInfos.find((userInfo: IUserInfo) => username === userInfo.username);
		if (userInfo !== undefined) {
			fetchDataForUser(userInfo);
		}
	}

    const editEventHandler: IEditEvents = {
        onEdited: async (id: String) => {
            // await fetchFoods();
        },
        onCancelled: (id: String) => {
        },
        onDeleted: async (id: String) => {
            // await fetchFoods();
        }
    };

	return (
		<>
			<form>
				<select
					onChange={selectHandler}
					className="form-select form-select-lg mb-3" aria-label=".form-select-lg example"
					defaultValue={myUserInfo?.username}
				>
					<option value={myUserInfo?.username}>{myUserInfo?.username}</option>

					{userInfos
						.filter((userInfo: IUserInfo) => {
							return userInfo.username !== myUserInfo?.username;
						})
						.map((userInfo: IUserInfo) => {
							return (
								<option key={userInfo.username} value={userInfo.username}>{userInfo.username}</option>
							);
						})}
				</select>
			</form>

			<AllFoods
				maxCaloriesPerDay={selectedUserInfo!.maxCaloriesPerDay}
				allFoods={foods}
				onEditEvent={editEventHandler}
			/>
		</>
	)
}