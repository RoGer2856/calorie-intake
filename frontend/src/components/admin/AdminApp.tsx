import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";
import { IUserInfo } from "../../model/UserInfo";
import PageNotFound from "../PageNotFound";
import AdminLayout from "./AdminLayout";
import ReportScreen from "./ReportScreen";

export default function AdminApp(props: {
    userInfo: IUserInfo
}): ReactElement {
    return (
        <>
            <AdminLayout userInfo={props.userInfo}>
                <Routes>
                    <Route path="/">
                        <Route index element={<ReportScreen />} />
                    </Route>
                    <Route path="*" element={<PageNotFound />} />
                </Routes>
            </AdminLayout>
        </>
    )
}