import MyLinks from "@/components/MyLinks";
import Link from "next/link";
export default function Header(){
    return(
        <header>
            <div className="container">
                <div className="title-and-links">
                    <Link href='/'> <h1>FPL Mini-League Board</h1> </Link>
                <MyLinks/>
                </div>
            </div>
        </header>
    )
}