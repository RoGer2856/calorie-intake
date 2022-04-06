import { ReactElement, useCallback, useEffect, useState } from "react";
import useApi from "../../hooks/use-api";
import { IGetFoodReportResponse } from "../../messages/Food";
import Loading from "../Loading";
import UseApiView from "../UseApiView";
import FoodReport from "./FoodReport";

export default function ReportScreen(): ReactElement {
    const [apiFeedback, api] = useApi();

    const [report, setReport] = useState<IGetFoodReportResponse | null>(null);

    const fetchFoodReport = useCallback(async function () {
        const response = await api.getFoodReport();
        if (response !== null) {
            const report = response as IGetFoodReportResponse;
            setReport(report);
        }
    }, [api]);

    useEffect(() => {
        fetchFoodReport();
    }, [fetchFoodReport])

    return (
        <>
            <p>Food report</p>

            <UseApiView apiFeedback={apiFeedback}>
                <>
                    {report === null
                        ?
                        <Loading />
                        :
                        <FoodReport foodReport={report} />
                    }
                </>
            </UseApiView>
        </>
    )
}