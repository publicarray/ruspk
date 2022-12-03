import React, { useState } from 'react';
import Table from './table';
// import { useRouter } from "next/router"
import Button from "./button";

export default function TablePaginate({url, columns, data, setData}) {
    const [pageIndex, setPageIndex] = useState(1);
    return (
        <div>
            <Table pageIndex={pageIndex} setPageIndex={setPageIndex} url={url} columns={columns} data={data} setData={setData}/>
            {/* <div style={{ display: 'none' }}><Table pageIndex={pageIndex + 1} setPageIndex={setPageIndex} url={url} columns={columns} data={data} setData={setData}/></div> */}
            <div className="flex mb-4">
                <Button className="mr-auto" onClick={() => setPageIndex(pageIndex - 1)}>Previous</Button>
                <Button className="ml-auto" onClick={() => setPageIndex(pageIndex + 1)}>Next</Button>
            </div>
        </div>
    )
}
