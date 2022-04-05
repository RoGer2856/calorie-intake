import { configureStore } from "@reduxjs/toolkit";
import { userInfoSliceReducer } from "./user-info";

const store = configureStore({
	reducer: {
		userInfo: userInfoSliceReducer,
	}
});

export default store;