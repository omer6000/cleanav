# cleanav
Cleaning Navigation System

This is a project of the "Hands-On Dependability" course.
For more details, check out the [task sheet](https://hod.cs.uni-saarland.de/projects/P01a.html).

sudo docker run --rm --cpus=1 --stop-timeout 180 -v $PWD/src/navigation.rs:/opt/cleanav/src/navigation.rs -w /opt/cleanav hod cargo test --release -v --test 'i*' --test 'cli'

docker run --rm --cpus=1 --stop-timeout 180 -v $PWD/src/analysis.rs:/opt/cleanav/src/analysis.rs -w /opt/cleanav hod cargo test --release -v --test 'markov_tests'