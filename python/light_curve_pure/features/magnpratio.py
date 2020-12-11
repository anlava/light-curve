from dataclasses import dataclass
from ._base import BaseFeature
from scipy.stats.mstats import mquantiles


@dataclass()
class MagnitudePercentageRatio(BaseFeature):
    n: float = 0.4
    d: float = 0.05

    def __call__(self, t, m, sigma=None, sorted=None, fill_value=None):
        n1, n2 = mquantiles(m, [self.n, 1 - self.n], alphap=0.5, betap=0.5)
        d1, d2 = mquantiles(m, [self.d, 1 - self.d], alphap=0.5, betap=0.5)
        return (n2 - n1) / (d2 - d1)


__all__ = ("MagnitudePercentageRatio",)