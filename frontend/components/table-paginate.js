import React, { useState } from 'react';
import Table from './table';
// import { useRouter } from "next/router"

export default function TablePaginate(props) {
    const [pageIndex, setPageIndex] = useState(1);
    return (
        <div>
            <Table pageIndex={pageIndex} url={props.url} columns={props.columns} data={props.data} setData={props.setData}/>
            <div style={{ display: 'none' }}><Table pageIndex={pageIndex + 1} url={props.url} columns={props.columns} /></div>
            <button className="py-1 px-3 text-white rounded-lg bg-blue-500 hover:bg-blue-700" onClick={() => setPageIndex(pageIndex - 1)}>Previous</button>
            <button className="py-1 px-3 text-white rounded-lg bg-blue-500 hover:bg-blue-700" onClick={() => setPageIndex(pageIndex + 1)}>Next</button>
        </div>
    )
}
