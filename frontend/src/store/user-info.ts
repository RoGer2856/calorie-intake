import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { IUserInfo } from "../model/UserInfo";

export interface IUserInfoState {
	userInfo: IUserInfo | null,
}

const initialUserInfoState = {
	userInfo: null,
};

const userInfoSlice = createSlice({
	name: "UserInfo",
	initialState: initialUserInfoState,
	reducers: {
		setUserinfo(state: IUserInfoState, action: PayloadAction<IUserInfo>) {
			state.userInfo = action.payload;
		},
	}
});

export const userInfoSliceReducer = userInfoSlice.reducer;
export const userInfoActions = userInfoSlice.actions;
