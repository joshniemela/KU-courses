import { apiUrl } from "../../../stores";
import { total_hours } from "../../../course";


// ungraded returns absent, total, fail and pass
function transform_ungraded_stats(stats) {
    return [
        { grade: "Fail", count: stats.fail },
        { grade: "Pass", count: stats.pass },
        { grade: "Absent", count: stats.absent },
    ];
}

function transform_graded_stats(stats) {
    return stats.grades.map((grade) => {
        return { grade: grade.grade, count: grade.count };
    });
}


function transform_stats(stats) {
    if (stats == null) {
        return null;
    } else {
        if (stats.grades == null) {
            return transform_ungraded_stats(stats);
        } else {
            return transform_graded_stats(stats);
        }
    }
}

function null_to_zero(grades) {
    // in each grade, count pair, if count is null, set it to 0
    if (grades == null) {
        return null;
    }
    return grades.map((grade) => {
        if (grade.count == null) {
            return { grade: grade.grade, count: 0 };
        } else {
            return grade;
        }
    });
}


export async function load({ fetch, params }) {
    const { courseId } = params;
    const API_URL = apiUrl();

    const res = await fetch(`${API_URL}/get-detailed-course-info?id=${courseId}`, {
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
        statistics: null_to_zero(transform_stats(course.statistics)),
        loading: false,
    };

}
