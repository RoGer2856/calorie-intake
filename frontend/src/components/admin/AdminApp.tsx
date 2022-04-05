import { ReactElement } from "react";
import { Navigate, Route, Routes } from "react-router-dom";
import { IUserInfo } from "../../model/UserInfo";
import MyFoods from "../food/MyFoods";
import PageNotFound from "../PageNotFound";
import AdminLayout from "./AdminLayout";
import AllUserFoods from "./AllUserFoods";
import ReportScreen from "./ReportScreen";

export default function AdminApp(props: {
    userInfo: IUserInfo
}): ReactElement {
    return (
        <>
            <AdminLayout userInfo={props.userInfo}>
                <Routes>
                    <Route path="/">
                        <Route index element={<Navigate replace to="/my-consumption" />} />
                        <Route path="/my-consumption" element={<MyFoods maxCaloriesPerDay={props.userInfo.maxCaloriesPerDay} />} />
                        <Route path="/everyones-consumption" element={<AllUserFoods />} />
                        <Route path="/report" element={<ReportScreen />} />
                    </Route>
                    <Route path="*" element={<PageNotFound />} />
                </Routes>
            </AdminLayout>
        </>
    )
}