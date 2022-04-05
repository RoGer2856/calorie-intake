import { ReactElement, useCallback, useEffect, useState } from "react";
import useApi from "../../hooks/use-api";
import { IGetFoodReportResponse } from "../../messages/Food";
import ErrorView from "../ErrorView";
import FoodReport from "./FoodReport";

export default function ReportScreen(): ReactElement {
    const api = useApi();

    const [report, setReport] = useState<IGetFoodReportResponse | null>(null);

    const fetchFoodReport = useCallback(async function () {
        const response = await api.getFoodReport();
        if (response !== null) {
            const report = response as IGetFoodReportResponse;
            setReport(report);
        }
    }, []);

    useEffect(() => {
        fetchFoodReport();
    }, [fetchFoodReport])

    return (
        <>
            <p>Food report</p>

            {api.errorMessage === null
                ?
                <>
                    {report === null
                        ?
                        <p>Loading...</p>
                        :
                        <FoodReport foodReport={report} />
                    }
                </>
                :
                <ErrorView errorMessage={api.errorMessage} />
            }
        </>
    )
}