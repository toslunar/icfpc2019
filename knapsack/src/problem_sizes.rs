use std::collections::HashMap;

type ProblemSizes = HashMap<String, (usize, usize)>;

const PROBLEM_SIZES: [(&str, (usize, usize)); 300] = [
    ("prob-001", (8, 3)),
    ("prob-002", (42, 43)),
    ("prob-003", (27, 37)),
    ("prob-004", (33, 63)),
    ("prob-005", (26, 39)),
    ("prob-006", (24, 23)),
    ("prob-007", (29, 29)),
    ("prob-008", (31, 33)),
    ("prob-009", (22, 26)),
    ("prob-010", (32, 55)),
    ("prob-011", (29, 29)),
    ("prob-012", (29, 29)),
    ("prob-013", (29, 29)),
    ("prob-014", (29, 29)),
    ("prob-015", (29, 29)),
    ("prob-016", (29, 29)),
    ("prob-017", (29, 29)),
    ("prob-018", (29, 29)),
    ("prob-019", (29, 29)),
    ("prob-020", (29, 29)),
    ("prob-021", (49, 49)),
    ("prob-022", (49, 49)),
    ("prob-023", (49, 49)),
    ("prob-024", (49, 49)),
    ("prob-025", (49, 44)),
    ("prob-026", (49, 49)),
    ("prob-027", (49, 49)),
    ("prob-028", (49, 49)),
    ("prob-029", (49, 49)),
    ("prob-030", (49, 45)),
    ("prob-031", (49, 49)),
    ("prob-032", (49, 49)),
    ("prob-033", (49, 49)),
    ("prob-034", (49, 49)),
    ("prob-035", (49, 49)),
    ("prob-036", (49, 49)),
    ("prob-037", (49, 49)),
    ("prob-038", (49, 49)),
    ("prob-039", (49, 49)),
    ("prob-040", (49, 49)),
    ("prob-041", (49, 49)),
    ("prob-042", (49, 49)),
    ("prob-043", (49, 49)),
    ("prob-044", (49, 49)),
    ("prob-045", (49, 49)),
    ("prob-046", (49, 49)),
    ("prob-047", (49, 49)),
    ("prob-048", (49, 49)),
    ("prob-049", (49, 49)),
    ("prob-050", (49, 49)),
    ("prob-051", (99, 100)),
    ("prob-052", (99, 100)),
    ("prob-053", (99, 100)),
    ("prob-054", (99, 86)),
    ("prob-055", (99, 100)),
    ("prob-056", (99, 100)),
    ("prob-057", (99, 100)),
    ("prob-058", (98, 100)),
    ("prob-059", (99, 100)),
    ("prob-060", (99, 100)),
    ("prob-061", (98, 100)),
    ("prob-062", (99, 100)),
    ("prob-063", (99, 100)),
    ("prob-064", (99, 100)),
    ("prob-065", (99, 100)),
    ("prob-066", (99, 100)),
    ("prob-067", (99, 100)),
    ("prob-068", (98, 100)),
    ("prob-069", (98, 100)),
    ("prob-070", (99, 100)),
    ("prob-071", (98, 96)),
    ("prob-072", (91, 99)),
    ("prob-073", (99, 98)),
    ("prob-074", (99, 99)),
    ("prob-075", (99, 87)),
    ("prob-076", (99, 99)),
    ("prob-077", (99, 99)),
    ("prob-078", (97, 98)),
    ("prob-079", (98, 98)),
    ("prob-080", (97, 99)),
    ("prob-081", (88, 97)),
    ("prob-082", (99, 99)),
    ("prob-083", (99, 99)),
    ("prob-084", (97, 99)),
    ("prob-085", (98, 99)),
    ("prob-086", (99, 98)),
    ("prob-087", (99, 99)),
    ("prob-088", (99, 99)),
    ("prob-089", (99, 99)),
    ("prob-090", (99, 99)),
    ("prob-091", (99, 99)),
    ("prob-092", (99, 99)),
    ("prob-093", (99, 97)),
    ("prob-094", (99, 99)),
    ("prob-095", (98, 99)),
    ("prob-096", (87, 99)),
    ("prob-097", (99, 99)),
    ("prob-098", (99, 99)),
    ("prob-099", (99, 98)),
    ("prob-100", (98, 99)),
    ("prob-101", (199, 200)),
    ("prob-102", (199, 200)),
    ("prob-103", (199, 199)),
    ("prob-104", (199, 190)),
    ("prob-105", (199, 200)),
    ("prob-106", (199, 200)),
    ("prob-107", (199, 143)),
    ("prob-108", (199, 200)),
    ("prob-109", (199, 200)),
    ("prob-110", (199, 200)),
    ("prob-111", (199, 200)),
    ("prob-112", (199, 200)),
    ("prob-113", (199, 164)),
    ("prob-114", (199, 200)),
    ("prob-115", (199, 200)),
    ("prob-116", (199, 200)),
    ("prob-117", (199, 167)),
    ("prob-118", (199, 200)),
    ("prob-119", (199, 200)),
    ("prob-120", (199, 200)),
    ("prob-121", (199, 199)),
    ("prob-122", (199, 199)),
    ("prob-123", (198, 199)),
    ("prob-124", (199, 199)),
    ("prob-125", (199, 196)),
    ("prob-126", (199, 198)),
    ("prob-127", (199, 195)),
    ("prob-128", (198, 199)),
    ("prob-129", (199, 199)),
    ("prob-130", (198, 198)),
    ("prob-131", (198, 198)),
    ("prob-132", (199, 198)),
    ("prob-133", (189, 193)),
    ("prob-134", (199, 194)),
    ("prob-135", (199, 199)),
    ("prob-136", (199, 199)),
    ("prob-137", (198, 197)),
    ("prob-138", (199, 199)),
    ("prob-139", (199, 194)),
    ("prob-140", (199, 199)),
    ("prob-141", (198, 198)),
    ("prob-142", (190, 195)),
    ("prob-143", (199, 199)),
    ("prob-144", (196, 196)),
    ("prob-145", (198, 199)),
    ("prob-146", (185, 197)),
    ("prob-147", (199, 198)),
    ("prob-148", (199, 199)),
    ("prob-149", (199, 199)),
    ("prob-150", (199, 199)),
    ("prob-151", (99, 100)),
    ("prob-152", (99, 100)),
    ("prob-153", (99, 100)),
    ("prob-154", (99, 94)),
    ("prob-155", (99, 91)),
    ("prob-156", (99, 100)),
    ("prob-157", (98, 100)),
    ("prob-158", (99, 100)),
    ("prob-159", (98, 100)),
    ("prob-160", (99, 100)),
    ("prob-161", (99, 99)),
    ("prob-162", (99, 99)),
    ("prob-163", (92, 94)),
    ("prob-164", (99, 98)),
    ("prob-165", (99, 97)),
    ("prob-166", (99, 98)),
    ("prob-167", (99, 99)),
    ("prob-168", (99, 99)),
    ("prob-169", (99, 99)),
    ("prob-170", (99, 99)),
    ("prob-171", (99, 99)),
    ("prob-172", (99, 99)),
    ("prob-173", (99, 99)),
    ("prob-174", (99, 99)),
    ("prob-175", (99, 99)),
    ("prob-176", (99, 99)),
    ("prob-177", (99, 99)),
    ("prob-178", (99, 99)),
    ("prob-179", (99, 99)),
    ("prob-180", (99, 99)),
    ("prob-181", (199, 200)),
    ("prob-182", (198, 199)),
    ("prob-183", (199, 200)),
    ("prob-184", (199, 200)),
    ("prob-185", (199, 200)),
    ("prob-186", (199, 200)),
    ("prob-187", (198, 200)),
    ("prob-188", (199, 200)),
    ("prob-189", (198, 199)),
    ("prob-190", (199, 200)),
    ("prob-191", (199, 199)),
    ("prob-192", (199, 199)),
    ("prob-193", (199, 199)),
    ("prob-194", (195, 199)),
    ("prob-195", (199, 199)),
    ("prob-196", (199, 197)),
    ("prob-197", (195, 199)),
    ("prob-198", (196, 197)),
    ("prob-199", (199, 199)),
    ("prob-200", (199, 199)),
    ("prob-201", (199, 199)),
    ("prob-202", (199, 199)),
    ("prob-203", (199, 186)),
    ("prob-204", (199, 199)),
    ("prob-205", (199, 199)),
    ("prob-206", (198, 199)),
    ("prob-207", (199, 199)),
    ("prob-208", (199, 199)),
    ("prob-209", (199, 197)),
    ("prob-210", (198, 199)),
    ("prob-211", (399, 400)),
    ("prob-212", (399, 400)),
    ("prob-213", (399, 400)),
    ("prob-214", (399, 400)),
    ("prob-215", (399, 400)),
    ("prob-216", (398, 399)),
    ("prob-217", (399, 392)),
    ("prob-218", (399, 399)),
    ("prob-219", (398, 399)),
    ("prob-220", (398, 397)),
    ("prob-221", (98, 99)),
    ("prob-222", (99, 92)),
    ("prob-223", (99, 100)),
    ("prob-224", (99, 100)),
    ("prob-225", (99, 100)),
    ("prob-226", (99, 100)),
    ("prob-227", (99, 93)),
    ("prob-228", (99, 100)),
    ("prob-229", (99, 100)),
    ("prob-230", (99, 100)),
    ("prob-231", (99, 99)),
    ("prob-232", (99, 99)),
    ("prob-233", (99, 99)),
    ("prob-234", (99, 99)),
    ("prob-235", (99, 95)),
    ("prob-236", (99, 99)),
    ("prob-237", (99, 99)),
    ("prob-238", (99, 99)),
    ("prob-239", (99, 99)),
    ("prob-240", (99, 99)),
    ("prob-241", (199, 200)),
    ("prob-242", (199, 200)),
    ("prob-243", (199, 200)),
    ("prob-244", (198, 200)),
    ("prob-245", (199, 200)),
    ("prob-246", (199, 199)),
    ("prob-247", (199, 200)),
    ("prob-248", (199, 200)),
    ("prob-249", (199, 200)),
    ("prob-250", (199, 200)),
    ("prob-251", (198, 199)),
    ("prob-252", (198, 199)),
    ("prob-253", (199, 196)),
    ("prob-254", (199, 198)),
    ("prob-255", (199, 199)),
    ("prob-256", (199, 198)),
    ("prob-257", (198, 199)),
    ("prob-258", (199, 176)),
    ("prob-259", (199, 199)),
    ("prob-260", (199, 199)),
    ("prob-261", (199, 198)),
    ("prob-262", (199, 199)),
    ("prob-263", (199, 199)),
    ("prob-264", (199, 198)),
    ("prob-265", (199, 198)),
    ("prob-266", (198, 187)),
    ("prob-267", (197, 198)),
    ("prob-268", (199, 197)),
    ("prob-269", (199, 199)),
    ("prob-270", (198, 197)),
    ("prob-271", (399, 400)),
    ("prob-272", (399, 400)),
    ("prob-273", (398, 399)),
    ("prob-274", (398, 400)),
    ("prob-275", (399, 319)),
    ("prob-276", (399, 400)),
    ("prob-277", (399, 400)),
    ("prob-278", (399, 400)),
    ("prob-279", (399, 400)),
    ("prob-280", (399, 225)),
    ("prob-281", (399, 400)),
    ("prob-282", (399, 400)),
    ("prob-283", (399, 400)),
    ("prob-284", (399, 400)),
    ("prob-285", (399, 400)),
    ("prob-286", (399, 399)),
    ("prob-287", (398, 398)),
    ("prob-288", (305, 359)),
    ("prob-289", (399, 399)),
    ("prob-290", (397, 392)),
    ("prob-291", (399, 398)),
    ("prob-292", (399, 399)),
    ("prob-293", (399, 399)),
    ("prob-294", (399, 398)),
    ("prob-295", (389, 398)),
    ("prob-296", (216, 369)),
    ("prob-297", (375, 387)),
    ("prob-298", (398, 351)),
    ("prob-299", (397, 398)),
    ("prob-300", (390, 399)),
];

pub fn get_problem_sizes_embedded() -> ProblemSizes {
    let mut problem_sizes = ProblemSizes::new();
    for (s, xy) in PROBLEM_SIZES.iter() {
        problem_sizes.insert(s.to_string(), *xy);
    }
    problem_sizes
}

pub fn get_problem_sizes_from_task_files() -> ProblemSizes {
    eprintln!("Retrieving problem sizes...");
    let path_patterns = [
        "../data/part-1-initial/*.desc",
        "../data/part-2-teleports/*.desc",
        "../data/part-3-clones/*.desc",
    ];

    let mut problem_sizes = HashMap::new();

    let mut x = 0;

    for path_pattern in &path_patterns {
        for path in glob::glob(path_pattern).unwrap() {
            let path_buf = path.unwrap();
            let problem_name = path_buf.file_stem().unwrap().to_str().unwrap().to_owned();
            let path_str = path_buf.to_str().unwrap().to_owned();
            let task = common::read_task(&path_str);

            if task
                .1
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&c| c == Some(common::Booster::X))
                        .any(|b| b)
                })
                .any(|b| b)
            {
                x += 1;
            }

            let (xsize, ysize) = common::get_xysize(&task.0);
            let (xsize, ysize) = (xsize - 2, ysize - 2); // TODO
            problem_sizes.insert(problem_name.clone(), (xsize, ysize));
        }
    }

    eprintln!("X: {}", x);

    eprintln!(
        "Sizes loaded for problems: {}\n{:?}",
        problem_sizes.len(),
        &problem_sizes
    );
    problem_sizes
}

pub fn get_problem_sizes() -> ProblemSizes {
    get_problem_sizes_embedded()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            get_problem_sizes_embedded(),
            get_problem_sizes_from_task_files()
        );
    }
}
