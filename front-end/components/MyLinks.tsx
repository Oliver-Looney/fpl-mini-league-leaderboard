import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faGithub, faLinkedin } from '@fortawesome/free-brands-svg-icons'
import Link from "next/link";

export default function MyLinks(){
    return(
        <div className="social">
            <div className="social-icons">
                <Link href='https://github.com/Oliver-Looney' target="_blank" aria-label="View my GitHub Profile">
                    <FontAwesomeIcon icon={faGithub} className="icon"/>
                </Link>
                <Link href='https://www.linkedin.com/in/oliver-looney-7332421bb/' target="_blank" aria-label="View my LinkedIn Profile">
                    <FontAwesomeIcon icon={faLinkedin} className="icon"/>
                </Link>
            </div>
            <div className="social-links">
                <Link href='https://www.oliverlooney.com/' className="social-link" target="_blank"> <h2>Portfolio Site</h2> </Link>
            </div>
        </div>
    )
}