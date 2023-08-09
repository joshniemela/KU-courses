import { apiUrl } from "../../../stores";
import { total_hours } from "../../../course";


export async function load({ fetch, params }) {
    const { courseId } = params;
    const API_URL = "http://localhost:3000/api";

    const res = await fetch(`${API_URL}/get-course?id=${courseId}`, {
        method: "GET",
        headers: {
            accept: "application/json",
            "Content-Type": "application/json",
        },
    });

    const course = await res.json();

    return {
        courseId: courseId,
        course: course,
        totalHours: total_hours(course),
        loading: false,
    };

}
